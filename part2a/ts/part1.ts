import {readFileSync} from 'fs';

type RPS = {
    move: string;
    winOrLose: Map<string, number>;
    val: number;
}




function read():void {
    const result = readFileSync("./input.txt", 'utf-8');
    const rock: RPS = {
        move: 'X',
        val: 1,
        winOrLose: new Map([['A', 3], ['B', 0], ['C', 6]])
    }
    const paper: RPS = {
        move: 'Y',
        val: 2,
        winOrLose: new Map([['A', 6], ['B', 3], ['C', 0]])
    }
    const scs: RPS = {
        move: 'Z',
        val: 3,
        winOrLose: new Map([['A', 0], ['B', 6], ['C', 3]])
    }
    let arr: RPS[] = [];
    arr.push(rock);
    arr.push(paper);
    arr.push(scs);
    let ans: number = 0;
    result.split(/\r?\n/).forEach(line =>  {
        const arry = line.split(" ");
        const op = arry[0];
        const me = arry[1].charCodeAt(0);
        ans += arr[me - 'X'.charCodeAt(0)].val + arr[me - 'X'.charCodeAt(0)].winOrLose.get(op);
    });
    console.log(ans);
}

read()