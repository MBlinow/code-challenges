/** 
 * https://adventofcode.com/2024/day/1
 * */

import fs from 'node:fs';
import * as path from 'path';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const main = () => {
    let contents = getFileContents();
    let {arr1, arr2} = parseAndSortInputArrays(contents)

    let result = calculateDistanceFromArrays(arr1, arr2);

    console.log(`Result is: ${result}`)
}

const getFileContents = () => {
    const __dirname = dirname(fileURLToPath(import.meta.url));
    const filePath = path.join(__dirname, 'input');
    const fileContents = fs.readFileSync(filePath, 'utf8');

    return fileContents;
}

const parseAndSortInputArrays = (input) => {
    const arr1 = []
    const arr2 = []

    let zoneIdPairs = input.split("\r\n")

    zoneIdPairs.forEach((pair) => {
        if (pair === ''){
            return;
        }
        let values = pair.split('   ')

        arr1.push(Number(values[0]));
        arr2.push(Number(values[1]));
    });


    arr1.sort((a, b) => a - b)
    arr2.sort((a, b) => a- b)

    return {arr1, arr2}
}

const calculateDistanceFromArrays = (arr1, arr2) => {
    let totalDistance = 0;

    for (let i = 0; i < arr1.length; i++){
        let dist = Math.abs(arr1[i] - arr2[i])
        //DEBUG console.log(`Adding distance ${dist}`)
        totalDistance += dist; 
    }

    return totalDistance;
}

main();