
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

struct node {
	int key;
	struct node *left, *right;
};

uint8_t* insert_c(uint8_t* nodeptr, int key);
uint8_t* insert_rust(uint8_t* nodeptr, int key);

int comp_main() {

    struct node node;
    struct node node_left;
    struct node node_right;
    node_left.left = NULL;
    node_left.right = NULL;
    node_right.left = NULL;
    node_right.right = NULL;
    node.left = &node_left;
    node.right = &node_right;

    int key;

    struct node* result_c = insert_c(&node, key);
    struct node* result_rust = insert_rust( &node, key);

    __CPROVER_assert(result_c->key == result_rust->key, "Are the results the same?");


}