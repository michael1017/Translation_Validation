#include <stdint.h>
#include <stdio.h>


void utoa_c(char *str, size_t v) {
    char *p = str;
    do {
        *p++ = '0' + (v % 10);
        v /= 10;
    } while (v > 0);
    size_t len = p - str;
    for (size_t i = 0; i < len / 2; i++) {
        char tmp = str[i];
        str[i] = str[len - i - 1];
        str[len - i - 1] = tmp;
    }
    str[len] = '\0';
    str[0] = '0';
}


int is_ascending_order_c(uint64_t *v, size_t n) {
    for (size_t j = 1; j < n; ++j)
        if (v[j - 1] >= v[j])
            return 0;

    return 1;
}

int mainc(){}