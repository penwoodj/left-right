DO NOT EDIT!!!

This is an exhaustive list of the types: Operator, Map, List, String, Boolean, Number, Undefined

Collection types are Maps Lists and Strings and all collections are iterable
Collections and Operators are reference types.
booleans, numbers and undefined are primatives

a key fact you might be missing is it is designed with compressed expression data flow readability with simplicity and complete correct expression as central with a simple easy to understand strategy. in other words simple semantics massive expressiveness. the operators expression execution sematics make this possible.  when I say the it is a left hungry curried operator oriented language that means that we evaluate on each operator and if it takes in a left arg when the expression the data it looks to the next thing so "data leftInputOperator" evaluates like "((data) leftInputOperator)" and a diatic operator operates the same way so "5 + 9" is evaluated like "(((5)+)9)"  so "((5)+)" returns an operator that takes a right argument then it tics to the next item in the expression 9 and the right input operator in the expression takes in the item to the right then executes and returns a number 14 and this is central to how we build expressions so "response@`body` /json" starts with response then the next tic item is @ which is a diatic operator so it takes in the item to the left in the expression which returns an unexecuted operator waiting for the right arg which is a string collection `body` which then evaluates the expression and outputes whatever is contained in the key "body" in the response map.  after the /json is an operator that mean cast to json. we evaluate with this ticking and hunger of operators before expression.


Our operator semantics work for the language sdk looks like this:
$ is the iterate or loop operator or the change each operator or the map function in JS and the prefix for other iterable operators
@ is the get operator and is the prefix for various get and navigation operations
? is the toBoolean or !! operator and is the prefix for other boolean output methods

/ is the divide operator and is semantically associated with conversion or type casting and is symbolic for moving forward and generating. 
| is the OR operator and is postfix to the operators related to sum types and stopping
& is the AND operator is prefix to the operators related to product types 
< is the less than operator and is semantically associated with multiple operators relating to the left direction and is semantically associated with separating
> is the greater than operator and is semantically associated with multiple operators relating to the right direction, along with condensing and grouping
^ is the exponent operator and is semantically associated with uppercase or wrapping
_ is the flatten operator or the floor operator or run expression without variable output if by itself before : UNLESS it is immediately preceeding without spaces the < or > sybmol OR UNLESS it is immediately preceeded or inside of by without spaces another operator symbol that has it.  and it is semantically associated with lowercase or flattening
+ is the addition concatination and spread operators depending on input types and is semantically associated with increasing
- is the subtract omit and remove operators depending on input types and is semantically associated with decreasing
% is the modulus operator and is semantically associated with ratios and percentages and is semantically associated with sorting
~ is unique and is semantically associated with randomness or entropy
" is the toString operator
# is size or length collections operator
= is type agnostic equality boolean operator
! is the negate to boolean or not operator

So the semantics combine within the sdk to expand the language into a fully express point free functional system using only the principals of the language and sticking with semantics and invariant representation with input type based execution behavior

Loop Operators
collection -> $ <- unexecuted operator with left arg being an item as it loops through the collection
collection of a list of maps -> $@ <- string of key to get from each item in the collection and returns list of item values OR list of strings to get from each map and returns a list of maps only with selected key value pairs
collection -> $? <- unexecuted operator with left arg and works like filter or if string it filters out any map in a list of maps where that string key name does not contain a truthy value or if a list of strings does the same as a single string but with and logic for all keys listed
$_ is flatmap
$~ is uniqueBy
$> is groupBy
$" is eachToString
$& is every or all
$| is some or any
$?| is find
$% is sort
$?! compact

String Operators 
" to string with alias /" doing the exact same thing
"_ to lower case
"^ to UpperCase
"^_ capitalize
"~ replace
<> split as long as not immediately preceeded without space by _
>< join as long as not immediately preceeded without space by _


Boolean Operators
| or
& and
= equals
?" isString
?# isNumber
?>< contains

reserved symbols that cannot appear in operators:
: is the assignment and conditional return operator and cannot appear in any operators at any position
, is the collection item separator operator and cannot appear in any operators at any position
. is the reverse arguments operator and cannot appear in any operators at any position
' is not yet delegated but is completely reserved for later and cannot appear in any operators at any position
() is precedence operators in expressions
[] is an empty list collection and the list encapsulation operators
{} is an empty map collection and both the operator and the map encapsulation operators
_< is the reserved left argument operator and no matter the context is always parse as left argument and evaluates to the value of the item passed in on the left
_> is the reserved right argument operator and no matter the context is always parse as right argument and evaluates to the value of the item passed in on the right
`` is an empty string collection and the encapsulation operator for both strings and string operators

general semantics:
data first/on the left in expressions

Operators 

there are 4 ways to make a non operator into an operator
Map Operators {} has 3: it has the left or right arg operator inside, it ends in an expression, or one or more of its keys is an expression
and String Operators `` if any of the interpolated elements in the {} contain a left or right argument

both Maps and string operators remain unexecuted unless the are either themselves declared in an expression that has all needed input around it as needed or if stored in a variable and put in an expression it executes

Keys in maps and map operators can be referenced as variables after the , at the end of it's value expression the exeptions to this are if the key is an expression in which case the operator returns what is to the right or executes a map operator to the right

