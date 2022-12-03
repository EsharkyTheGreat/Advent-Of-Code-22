import {readFileSync} from 'fs';

type RPS = {
    move: string;
    winOrLose: Map<string, number>;
    val: number;
}




function read():void {
    const result = readFileSync("./input.txt", 'utf-8');
    const loss: RPS = {
        move: 'X',
        val: 0,
        winOrLose: new Map([['A', 3], ['B', 1], ['C',  2]])
    }
    const draw: RPS = {
        move: 'Y',
        val: 3,
        winOrLose: new Map([['A', 1], ['B', 2], ['C', 3]])
    }
    const win: RPS = {
        move: 'Z',
        val: 6,
        winOrLose: new Map([['A', 2], ['B', 3], ['C', 1]])
    }
    let arr: RPS[] = [];
    arr.push(loss);
    arr.push(draw);
    arr.push(win);
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