#include <string.h>

char * c_url_get_path(char *url);
char * url_get_path(char *url);

int main(void) {

    unsigned long int len = 11;
    unsigned char url[len];

    url[len - 1] = '\0';

    char* res1 = c_url_get_path(url);
    char* res2 = url_get_path(url);

    int len1 = strlen(res1);
    int len2 = strlen(res2);

    __CPROVER_assert(len1 == len2, "Programs have equal output length");
}