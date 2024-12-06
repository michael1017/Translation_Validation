#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

struct node {
	int key;
	struct node *left, *right;
};

// A utility function to create a new BST node
struct node* newNode(int item)
{
	struct node* temp
		= (struct node*)malloc(sizeof(struct node));
	temp->key = item;
	temp->left = NULL;
	temp->right = NULL;
	return temp;
}

uint8_t* insert_c(uint8_t* nodeptr, int key)
{
	struct node* node = (struct node*)nodeptr;
	/* If the tree is empty, return a new node */
	if (node == NULL)
		return (uint8_t*) newNode(key);

	/* Otherwise, recur down the tree */
	if (key < node->key)
		node->left = (struct node*) insert_c( (uint8_t*) node->left, key);
	else
		node->right = (struct node*) insert_c((uint8_t*) node->right, key);

	/* return the (unchanged) node pointer */
	return (uint8_t*) node;
}

void main(){
	
}
