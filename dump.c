#define _GNU_SOURCE
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/mtio.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>

int set_tape_partition(int fd, int partition) {
    struct mtop mt_command;

    // Prepare the MTSETPART command to switch to the desired partition
    mt_command.mt_op = MTSETPART;
    mt_command.mt_count = partition;

    // Execute the partition switch
    if (ioctl(fd, MTIOCTOP, &mt_command) < 0) {
        perror("MTSETPART failed");
        exit(1);
    }

    return 0;
}

int get_current_lbn(int fd) {
    struct mtpos mt_position;

    // Query the current tape position (LBN)
    if (ioctl(fd, MTIOCPOS, &mt_position) < 0) {
        perror("MTGETPOS failed");
        exit(1);
    }

    // Return the current logical block number
    return mt_position.mt_blkno;
}

int set_scsi_opt(int fd, int option) {
    struct mtop mt_command;

    // Prepare the MTSETOPTIONS command to set scsi2logical mode
    mt_command.mt_op = MTSETDRVBUFFER;
    mt_command.mt_count = MT_ST_SETBOOLEANS;
    mt_command.mt_count |= option;

    // Execute the command
    if (ioctl(fd, MTIOCTOP, &mt_command) < 0) {
        perror("MTSETOPTIONS failed");
        exit(1);
    }
}

int scsi_rewind(int fd) {
    struct mtop mt_command;

    mt_command.mt_op = MTREW;
    mt_command.mt_count = 1;

    // Execute the command
    if (ioctl(fd, MTIOCTOP, &mt_command) < 0) {
        perror("MTSETOPTIONS failed");
        exit(1);
    }
}

void read_to_file(int fd, char *filename) {
    int outfd = open(filename, O_WRONLY | O_CREAT);
    char buf[512 * 1024];

    for (;;) {
        int rc = read(fd, buf, sizeof(buf));
        if (rc < 0) {
            perror("read");
            exit(1);
        }
        if (rc == 0) {
            break;
        }
        rc = write(outfd, buf, rc);
        if (rc < 0) {
            perror("write");
            exit(1);
        }
    }

    close(outfd);
}

int main(int argc, char **argv) {
    int fd = open(argv[1], O_RDWR);
    if (fd < 1) {
        perror("open");
        exit(1);
    }
    // Make sure we start at the beginning
    scsi_rewind(fd);

    // Enable logical blocks so we can get the block position of each extent
    set_scsi_opt(fd, MT_ST_SCSI2LOGICAL);
    // Enable partitions so we can switch to the data partition
    set_scsi_opt(fd, MT_ST_CAN_PARTITIONS);
    set_tape_partition(fd, 1);

    // First file on tape is the volume label
    read_to_file(fd, "volume-label");
    // Followed by the LTFS label
    read_to_file(fd, "ltfs-label");
    // Then an empty file
    read_to_file(fd, "/dev/null");

    // The rest of the tape will be an index file followed by a data extent
    for (int i=0;; i++) {
        char *fn;
        // Name the indices sequentially
        asprintf(&fn, "index-%d", i);
        read_to_file(fd, fn);
        free(fn);

        // Name the data based on its logical block, for later reconstruction
        int lbn = get_current_lbn(fd);
        asprintf(&fn, "data-%d", lbn);
        read_to_file(fd, fn);
        free(fn);
    }

    return 0;
}

