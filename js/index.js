// (x & 1) ^ (y & 1)
// (x ^ y) & 1

/*
n & 1 => tests if the number is odd
true = 1
false = 0

0 1 0 1 (5)
& & & &
0 0 0 1 (1)
= = = =
0 0 0 1

*/

// for(let y = 0; y < 8; y++)
//     for(let x = 0; x < 8; x++) {
        
//     }

function range(n) {
    return [...Array(n).keys()];
}

for(let y = 0; y < 8; y++) {
    console.log(range(8).map((x) => (x & 1) ^ (y & 1)).join(' '))
}
