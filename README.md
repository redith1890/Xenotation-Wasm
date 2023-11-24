# Xenotation
https://zerowanderer.github.io/Xenotation/
Transcribe from Arabic numerals to xenotation

Inspired by the work of the CCRU, more specifically by Dr Barker's Tic Xenotation  
  
Xenotation is a notation by which any natural number can be expressed without the need for the use of sequentiality or a null positional marker (in Arabic numerals 0). This notation only has 2 symbols. "()" and "·"  

The "·" represents the first prime number "2". All symbols are multiplied regardless of the order. Therefore, the powers of 2 will be expressed as <br>
<br>
2^1= · <br>
2^2= ·· <br>
2^3=  ··· <br>
<br>
For the other numbers, "()" will be implemented, which converts what is inside into a prime, whose ordinal position in the list of primes (its index) will be what is inside that parenthesis. Hence: <br>
<br>
(·) = second prime = 3 <br>
(·)(·) = second prime x second prime = 3x3 = 9 <br>
((·)) = third prime = 5 <br>
<br>
To calculate a number that is not prime and not a power of 2, it will be decomposed into primes using the fundamental theorem of arithmetic. <br>
