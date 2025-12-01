// https://adventofcode.com/2025/day/1

import fs from 'node:fs';
import * as path from 'path';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

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
         
        let integerPart = parseInt(line.replace(/\D/g, '')); // Removes all non-digits

        if (line[0] === 'R'){
            currentCounter += integerPart;
        }
        else {
            currentCounter -= integerPart;
        }

        if (currentCounter % 100 === 0){
            console.log(`Found a multiple of 100: ${currentCounter}`);
            foundZeroes += 1;
        }
        // console.log(`iteration: ${iterations}. Move: ${line}. Current counter is: ${currentCounter}`);
    }

    return foundZeroes;
}

main();