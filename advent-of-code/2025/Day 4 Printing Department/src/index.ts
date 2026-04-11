// https://adventofcode.com/2025/day/4

import fs from 'node:fs';
import * as path from 'path';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const main = () => {
    const fileContents = getFileContents();
    const lines = fileContents.split('\n');

    console.log(lines);
}

const getFileContents = () => {
    const __dirname = dirname(fileURLToPath(import.meta.url));
    const filePath = path.join(__dirname, 'input');
    const fileContents = fs.readFileSync(filePath, 'utf8');

    return fileContents;
}

main();