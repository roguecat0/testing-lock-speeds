#include <stdio.h>
#include <pthread.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/time.h>
#include <stdatomic.h>

atomic_int keep_running = 0;
int data  __attribute__((aligned(64))) = 10; 
pthread_mutex_t lock;

__attribute__((optimize("O0")))
void *thread_logic(void* args){
    pthread_mutex_t* lockaddr = &lock;
    uint64_t reads = 0;
    while(!atomic_load(&keep_running));
    while(atomic_load(&keep_running)){
        pthread_mutex_lock(lockaddr);
        int internalData = data;
        reads++;
        pthread_mutex_unlock(lockaddr);
    }
    uint64_t *result = malloc(sizeof(uint64_t));
    *result = reads;
    return result;
}

float mutex_reads(uint16_t num_readers, uint32_t run_time){
    pthread_mutex_init(&lock, NULL);
    printf("\nReaders : %d\nTime : %d ms\n", num_readers, run_time);

    pthread_t* threads = (pthread_t*)calloc(num_readers, sizeof(pthread_t));

    for(uint16_t i=0; i<num_readers; i++){
        pthread_create(&threads[i], NULL, thread_logic, NULL);
    }


    struct timeval start, end;
    gettimeofday(&start, NULL);
    gettimeofday(&end, NULL);
    long elapsed_time = 0;

    keep_running = 1;
    printf("start\n");
    run_time *= 1000;
    
    while(elapsed_time < run_time){
        gettimeofday(&end, NULL);
        elapsed_time = (end.tv_sec - start.tv_sec) * 1000000L + (end.tv_usec - start.tv_usec);
    }
    keep_running = 0;
    printf("stop\n");
    uint64_t total_reads = 0;
    for(uint16_t i=0; i<num_readers; i++){
        uint64_t *reads;
        pthread_join(threads[i], (void**) &reads);
        total_reads += *reads;
        free(reads);
    }
    printf("Total Reads : %ld\n", total_reads);
    printf("Reads per microsecond : %f\n", total_reads/(float)run_time);
    pthread_mutex_destroy(&lock);
    free(threads);
    
    return total_reads/(float)run_time;
}


int main(int argc, char *argv[]){
    if(argc != 3){
        printf("usage: %s <num_reader_threads> <run_time>\n", argv[0]);
        exit(0);
    }

    mutex_reads(atoi(argv[1]), atoi(argv[2]));
}