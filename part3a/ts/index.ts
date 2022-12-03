import {readFileSync} from 'fs';



function read():void {
    const result = readFileSync("./input.txt", 'utf-8');
    let ans:number = 0;
    result.split(/\r?\n/).forEach(line =>  {
        let map = new Map<string, boolean>();
        let i =0, j = (line.length/2);
        while(i != line.length/2){
            map.set(line[i],true) 
            i++;
        }
        let char: string = 'A';
        while(j != line.length){
            if(map.get(line[j]) != undefined){
                char = line[j];
                break;
            }
            j++;
        }
        if(char[0].toUpperCase() == char[0]){
            ans += char[0].charCodeAt(0) - 'A'.charCodeAt(0) + 27;
        }else{
            ans += char[0].charCodeAt(0) - 'a'.charCodeAt(0) + 1;
        }
      });
    console.log(ans);
}

read()