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
    if (!grid || !grid[0]?.length) {
        console.log('!Grid is empty!');
        return 0;
    }

    const gridWidth = grid[0].length;
    const gridHeight = grid.length; 

    let totalValid = 0;


    let debugLineBuffer = '';

    let totalChars = gridWidth * gridHeight;

    let rollsRemoved = true;

    let removalIterations = 0;

    while (rollsRemoved){
        rollsRemoved = false;
        removalIterations++;
        let removedRolls = 0;


        let pVert = 0;
        let pHorz = 0;
        for ( let i = 0; i < totalChars; i++ ){

            
            let curRow = grid[pVert];
            let curChar: string = 'F';


            // Handle non-@ chars
            if (curRow){
                curChar = curRow[pHorz] ?? '';
            }

            if (curChar === "." || curChar === "F") {
                debugLineBuffer += curChar;
            }
            else {

                if (!grid[pHorz]?[pVert] : false)continue;

                const isValid = pointIsValid(pHorz, pVert, grid);
                if (isValid) {
                    totalValid++;

                    rollsRemoved = true;
                    removedRolls++;
                    grid[pVert]![pHorz] = "."; //Remove @ from grid for future iterations
                }

                if (isValid) debugLineBuffer += 'x'
                else debugLineBuffer += curChar;
            }

            if (pHorz < (gridWidth -1)) pHorz++;
            else {
                debug(`${debugLineBuffer} \n`);

                pVert++;
                pHorz = 0;

                if (pVert >= gridHeight -1) break;

                debugLineBuffer = '';
                
            } 
        }
        debug(`removed Rolls: ${removedRolls}`);
        console.log(`Removal Iterations: ${removalIterations} \n`)
    }

    return totalValid;
}

//4046 too high
//1533 too high

const pointIsValid = (x: number, y: number, grid: Grid): boolean => {
    let collisions = 0;

    //get three rows, 3 characters width around target
    //append three rows (9 char total) 
    //count rollChars in collection.  
    //Allow 9 chars (including target point which isn't counted against required 8);
    let checkSquareOriginX = x - 1;
    let checkSquareOriginY = y - 1;

    for (let i = 0; i < 3; i++){
        collisions += countRowChunk(checkSquareOriginX, grid[checkSquareOriginY]);
        //checkSquareOriginX++;
        checkSquareOriginY++;
    }

    //fewer than 4
    //3 adjecent points plus target point for 4.
    if (collisions <= 4) return true;
    return false;
}

const countRowChunk = (startIndex: number, gridRow: string[] | undefined): number => {
    const rollChar = TARGET_CHAR;
    let charCount = 0;

    if (!gridRow) {
        //console.log(`returning, row doesn't exist`);
        return 0; //Returns early for yAxis -1 or length over 1
    }

    let offset = 0;
    if (startIndex < 0) offset = 1;
    
    const chunkString = gridRow.slice(startIndex + offset, startIndex +3);

    //console.log(`Counting chunk ${chunkString}`);

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

const assertEqual = (a:unknown, b: unknown) =>{
    if (a !== b){
        console.log(`warning!!`);
    }
}

const debug = (mes: string) => {
    const DEBUG = true 
    //false
    if (DEBUG) console.log(mes);
}

main();