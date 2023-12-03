import fs from "fs";

const numbers = [
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine",
];

const input = fs.readFileSync("day1/input-2.txt").toString().split("\n");

let answer = 0;

for (const line of input) {
  const digits: string[] = [];

  line.split("").forEach((char, lineIndex) => {
    if (parseInt(char)) {
      digits.push(char);
    }

    numbers.forEach((number, index) => {
      if ((line as string).slice(lineIndex).startsWith(number)) {
        digits.push((index + 1).toString());
      }
    });
  });

  const score = parseInt(digits[0] + digits[digits.length - 1]);
  answer += score;
}

console.log(answer);
