import * as fs from 'fs'
import * as readline from 'readline'


const file = readline.createInterface({
    input: fs.createReadStream('assets/day2.txt')
  });
  
const data: string[] = []

  file.on('line', (line) => {
      data.push(line)
  })

  file.on('close', () => {
    let pos = 0;
    let depth = 0;
    let aim = 0;
    data.forEach((v, idx) => {
        const r = v.split(' ')
        switch(r[0]) {
            case "forward":
                pos += +r[1]
                depth += aim * +r[1]
            break;
            case "up":
                aim -= +r[1]
            break;
            case "down":
                aim += +r[1]
            break;
        }
    })
    console.log(pos, depth, pos * depth)
  });