// https://adventofcode.com/2025/day/4

import fs from 'node:fs';
import * as path from 'path';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

type Grid = string[][];

const TARGET_CHAR = "@";
const INPUT_FILENAME = 'inputTest';

const main = () => {
    const grid = getData();
    const result = scanGrid(grid);
    console.log(result);
}

const scanGrid = (grid: Grid): number  => {
    if (!grid[0]?.length) {
        console.log('!Grid is empty!');
        return 0;
    }

    const gridWidth = grid[0].length;
    const gridHeight = grid.length; 

    let totalValid = 0;
    let py = 0;
    let px = 0;

    let debugBuffer = '';

    let totalChars = gridWidth * gridHeight;
    for ( let i = 0; i < totalChars; i++ ){
        let curRow = grid[py];
        let curChar: string = 'F';

        // Handle non-@ chars
        if (curRow){
            curChar = curRow[px] ?? '';
        }

        if (curChar === "." || curChar === "F") {
            debugBuffer += curChar;
        }
        else {
            const isValid = pointIsValid(px, py, grid);
            if (isValid) totalValid++;

            if (isValid) debugBuffer += 'x'
            else debugBuffer += curChar;
        }

        if (px < gridWidth) px++;
        else {
            console.log(`${debugBuffer}`)
            py++;
            px = 0;

            debugBuffer = '';
        } 
    }

    return totalValid;
}

//4046 too high

const pointIsValid = (x: number, y: number, grid: Grid): boolean => {
    let collisions = 0;

    //get three rows, 3 characters width around target
    //append three rows (9 char total) 
    //count rollChars in collection.  
    //Allow 9 chars (including target point which isn't counted against required 8);
    let checkSquareOriginX = x - 1;
    let checkSquareOriginY = y + 1;

    for (let i = 0; i < 3; i++){
        collisions += countRowChunk(checkSquareOriginX, grid[checkSquareOriginY]);
        checkSquareOriginX++;
        checkSquareOriginY--;
    }

    //4 adjecent points plus target point for 5.
    if (collisions <= 5) return true;
    return false;
}

const countRowChunk = (startIndex: number, gridRow: string[] | undefined): number => {
    const rollChar = TARGET_CHAR;
    let charCount = 0;

    if (!gridRow) return 0;

    let offset = 0;
    if (startIndex < 0) offset = 1;
    
    const chunkString = gridRow.slice(startIndex + offset, startIndex +3);

    chunkString.map(char => {
        if (char === rollChar) charCount++;
    });

    //input leftmost x index and row
    return charCount;
}

const getData = (): Grid => {
    const fileContents = getFileContents();
    const lines = fileContents.split('\n');
    const grid = gridFromLines(lines);
    return grid;
}

const gridFromLines = (lines: string[]): Grid => {
    const twoDGrid: string[][] = [];
    lines.forEach(line => {
        let trimmedLine = line.replace("\r", "");
        const lineArray = trimmedLine.split('');
        twoDGrid.push(lineArray);
    });

    return twoDGrid;
}

const getFileContents = () => {
    const __dirname = dirname(fileURLToPath(import.meta.url));
    const filePath = path.join(__dirname, INPUT_FILENAME);
    const fileContents = fs.readFileSync(filePath, 'utf8');

    return fileContents;
}

main();