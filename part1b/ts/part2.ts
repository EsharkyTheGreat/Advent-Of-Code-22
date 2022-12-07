import {readFileSync} from 'fs';


function read():void {
    const result = readFileSync("./input.txt", 'utf-8');
    let maxi:number = 0;
    let ans: number[] = [];
    result.split(/\r?\n/).forEach(line =>  {
        let curr: number = parseInt(line);
        if(Number.isNaN(curr)){
            ans.push(maxi);
            maxi = 0;
        }else{
            maxi += curr;
        }
      });
    ans.push(maxi);
    ans.sort((a,b)=> b - a);
    console.log(ans[0] + ans[1] + ans[2]);
}

read()