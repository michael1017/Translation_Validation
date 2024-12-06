
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Node
{
    unsigned int key;
    struct Node *left;
    struct Node *right;
    unsigned int height;
} Node;

struct Node *leftRotate(struct Node *x);
struct Node *left_rotate(struct Node *x);

int comp_main() {

    Node node;
    Node node_left;
    Node node_right;
    Node node_left_left;
    Node node_left_right;
    Node node_right_left;
    Node node_right_right;
    node_left.left = &node_left_left;
    node_left.right = &node_left_right;
    node_right.left = &node_right_left;
    node_right.right = &node_right_right;
    node.left = &node_left;
    node.right = &node_right;

    Node *result_c = leftRotate(&node);
    Node *result_rust = left_rotate(&node);

    __CPROVER_assert(result_c->key == result_rust->key, "Are the results the same?");


}