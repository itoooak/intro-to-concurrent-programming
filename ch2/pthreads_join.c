#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREADS 10

void *thread_func(void *arg) {
    int id = (int)arg;
    for (int i = 0; i < 5; i++) {
        printf("id = %d, i = %d\n", id, i);
        sleep(1);
    }

    return "finished!";
}

int main(int argc, char *argv[]) {
    pthread_t v[NUM_THREADS];
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&v[i], NULL, thread_func, (void *)i) != 0) {
            perror("pthread_create");
            return -1;
        }
    }

    for (int i = 0; i < NUM_THREADS; i++) {
        char *ptr;
        if (pthread_join(v[i], (void **)&ptr) == 0) {
            printf("msg = %s\n", ptr);
        } else {
            perror("pthread_create");
            return -1;
        }
    }

    return 0;
}
