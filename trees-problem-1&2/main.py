'''
Morgan Purcell
November 30th, 2022
A program to construct a binary tree and store characters in lexiographic order
'''

#define the Treenode class
class Treenode:
    #define the constructor which takes the value of a tree node as a parameter
    def __init__(self, value):
        #initialize the attributes for a tree node
        self.value = value
        self.left = None
        self.right = None

    #define the insert function to insert a node into the tree
    def insert(self, value):
        #check if the value to insert is less than the current nodes value
        if value < self.value:
            #check if the current node has no left child
            if not self.left:
                #construct a new node
                self.left = Treenode(value)
            #else, make a recursive call on the left child
            else:
                self.left.insert(value)
        #check if the value to insert is greater than the current nodes value
        else:
            #check if the current node has no right child
            if not self.right:
                #construct a new node
                self.right = Treenode(value)
            #else, make a recursive call on the right child
            else:
                self.right.insert(value)

    #define the inorder traversal function to print the tree (as left as possible)
    def inorder_trav(self):
        #check if the current node has a left child
        if self.left:
            #make a recursive call on the left child
            self.left.inorder_trav()
        #print the current nodes value
        print(self.value)
        #check if the current node has a right child
        if self.right:
            #make a recursive call on the right child
            self.right.inorder_trav()

    #define the preorder traversal function to print the tree (print immediately)
    def preorder_trav(self):
        #print the current nodes value
        print(self.value)
        #check if the current node has a left child
        if self.left:
            #make a recursive call on the left child
            self.left.preorder_trav()
        #check if the current node has a right child
        if self.right:
            #make a recursive call on the right child
            self.right.preorder_trav()

    #define the postorder traversal function to print the tree (as deep as possible)
    def postorder_trav(self):
        #check if the current node has a left child
        if self.left:
            #make a recursive call on the left child
            self.left.postorder_trav()
        #check if the current node has a right child
        if self.right:
            #make a recursive call on the right child
            self.right.postorder_trav()
        #print the current nodes value
        print(self.value)

    #defind the find function to search for a node
    def find(self, value):
        #check if the value is less than the current node
        if value < self.value:
            #check if the current node has no left child
            if not self.left:
                #call the insert function to create a new node with the value
                self.insert(value)
                return
            else:
                #else, make a recursive call on the right child 
                return self.left.find(value)

        #check if the value is greater than the current node
        elif value > self.value:
            #check if the current node has no right child
            if not self.right:
                #call the insert function to create a new node with the value
                self.insert(value)
                return
            else:
                #else, make a recursive call on the right child
                return self.right.find(value)

        #if we reach this point, the symbol was foudn in the tree
        else:
            print("The symbol was found")
            return
        
        
#define main
def main():
    #initalize a list of chars
    items = ['D', 'i', 's', 'c', 'r', 'e', 't', '=', 'f', 'u', 'n']
    #create a tree object, lexiTree, using the first char in the items list
    lexiTree = Treenode(items[0])
    #remove the first char in the items list
    items.pop(0)
    
    #iterate through the items list and insert values into the lexiTree binary tree
    for i in items:
        lexiTree.insert(i)

    #prompt for an ascii symbol to insert or locate in the tree
    symbol = input("Enter an ASCII Symbol: ")
    lexiTree.find(symbol)

    #promt the user to display the tree
    display = input("\nChoose how to display the tree:\n1.Inorder\n2.Preorder\n3.Postorder\n")
    print("\nThe tree:")
    if display == '1':
        lexiTree.inorder_trav()
    elif display == '2':
        lexiTree.preorder_trav()
    elif display == '3':
        lexiTree.postorder_trav()

#call the main function
if __name__ == '__main__':
    main()
