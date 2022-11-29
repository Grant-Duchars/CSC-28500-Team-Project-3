/*Name: Romyal Swarnkar
Team: Discrete Bois
Course: Discrete structures
Question 4 - Prefix calculator*/

#include <iostream>
#include <stack>
#include <math.h>

using namespace std;

//Prefix calculator function
int calculator(string c1) {
    stack <int> s;                                      //using a stack
    
     for (int i = c1.length()-1; i >= 0; i--) {         //starting from the last digit
        
        if (c1.at(i) >= '0' && c1.at(i) <= '9') {       //If it's an operand, I push it in the stack
            s.push(c1.at(i) - 48);
        }
        else {                                          //If it's an operator, then I pop the numbers off of the stack and save it in variables.
            int num1 = s.top();
            s.pop();
            int num2 = s.top();
            s.pop();
            
            if (c1.at(i) == '+') {                      //Then I push the numbers on the stack again after performing the required calculation
                s.push(num1+num2);
             }
            else if (c1.at(i) == '-') {
                 s.push(num1-num2);
             }
            else if (c1.at(i) == '*') {
                 s.push(num1*num2);
             }
            else {
                 s.push(num1/num2);
             }
         }
     }
    
    return s.top();                                     //At the end, the only number remaining in the stack is the answer
}

int main(int argc, char **argv)
{
    string d = "-+7*45+20";                             //A prefix expression
    
    cout << calculator(d);
	return 0;
}
