import {readFileSync} from 'fs';


function read():void {
    const result = readFileSync("./input.txt", 'utf-8');
    let maxi:number = 0;
    let ans:number = 0;
    result.split(/\r?\n/).forEach(line =>  {
        let curr: number = parseInt(line);
        if(Number.isNaN(curr)){
            ans = (maxi > ans ? maxi: ans);
            maxi = 0;
        }else{
            maxi += curr;
        }
      });
    console.log((maxi > ans ? maxi: ans));
}

read()