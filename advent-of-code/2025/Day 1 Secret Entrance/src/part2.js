// https://adventofcode.com/2025/day/1#part2

import fs from 'node:fs';
import * as path from 'path';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

import { getZeroesInRange } from './utills.js';

function main(){
    let inputs = getProblemInput();
    let result = processInputs(inputs);

    console.log(`The result is: ${result}`);
}

const getProblemInput = () => {
    const __dirname = dirname(fileURLToPath(import.meta.url));
    const filePath = path.join(__dirname, 'input');
    const fileContents = fs.readFileSync(filePath, 'utf8');

    let inputArray = fileContents.split('\n');

    return inputArray;
}

function processInputs(inputs){
    let currentCounter = 50;
    let foundZeroes = 0;

    for (let line of inputs){
        let itStart = currentCounter;
         
        let integerPart = parseInt(line.replace(/\D/g, '')); // Removes all non-digits

        if (line[0] === 'R'){
            currentCounter += integerPart;
        }
        else {
            currentCounter -= integerPart;
        }

        let itEnd = currentCounter;
        
        let rangeZeroes = getZeroesInRange(itStart, itEnd);
        foundZeroes += rangeZeroes;

        // if (currentCounter % 100 === 0){
        //     // console.log(`Found a multiple of 100: ${currentCounter}`);
        //     foundZeroes += 1;
        // }
        // console.log(`iteration: ${iterations}. Move: ${line}. Current counter is: ${currentCounter}`);
    }

    return foundZeroes;
}

// function getZeroesInRange(start, end){
//     let zeroesFound = 0;

//     let incrementor = start < end ? 1 : -1;
//     for (let i = start; incrementor > 0 ? i <= end : i >= end; i += incrementor){
//         if (i % 100 === 0){
//             zeroesFound += 1;

//             // Calculate remaining zeroes, no need to iterate further
//             let remainingRange = Math.abs(end - i);
//             let additionalZeroes = Math.floor(remainingRange / 100);
//             zeroesFound += additionalZeroes;
//             break;
//         }
//     }
//     return zeroesFound;
// }

main();