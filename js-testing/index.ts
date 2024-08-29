import * as fs from 'fs';
import * as path from 'path';

console.log("hello");

const file: string = path.join(__dirname, 'ascii.txt');
const content: string[] = fs.readFileSync(file, 'utf8').split('\n');
const encodedFile: string = path.join(__dirname, 'ascii_encoded.txt');

content.forEach((line: string) => {
    const encoded: string = encodeURIComponent(line);
    fs.appendFileSync(encodedFile, encoded + '\n', 'utf8');
});

