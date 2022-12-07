import {readFileSync} from 'fs';

type Pre = {
    one: boolean,
    two?: boolean,
}

function read():void {
    const result = readFileSync("./input.txt", 'utf-8');
    let ans:number = 0, j = 0;
    let map = new Map<string, Pre>();
    let char: string = 'A';
    result.split(/\r?\n/).forEach(line =>  {
        if(j % 3 == 0){
            map = new Map<string, Pre>();
            let i =0;
            while(i != line.length){
                map.set(line[i], {one:true}) 
                i++;
            }
        }else if(j % 3 ==1){
            let i =0;
            while(i != line.length){
                if(map.has(line[i])){
                    map.set(line[i], {one:true, two: true});
                }
                i++;
            }
        }else{
            let i =0;
            while(i != line.length){
                if(map.has(line[i])){
                    let obj = map.get(line[i]);
                    if(obj.one && obj.two){
                        char = line[i];
                    }
                }
                i++;
            }
            if(char[0].toUpperCase() == char[0]){
                ans += char[0].charCodeAt(0) - 'A'.charCodeAt(0) + 27;
            }else{
                ans += char[0].charCodeAt(0) - 'a'.charCodeAt(0) + 1;
            }
        }
        j++;
      });
    console.log(ans);
}

read()