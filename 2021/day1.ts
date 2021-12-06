import * as fs from 'fs'
import * as readline from 'readline'


const file = readline.createInterface({
    input: fs.createReadStream('assets/day1.txt')
  });
  
const data: number[] = []

  file.on('line', (line) => {
      data.push(+line)
  })

  file.on('close', () => {
      let higher = 0
    // console.log('Total', data.length)

    const arr: number[] = []
    data.forEach((val, idx) => {
        if(data[idx+1] && data[idx+2]) {
        const result = data[idx] + data[idx+1] + data[idx+2]
        // console.log(result)
        arr.push(result)
        }
    })
    // console.log(arr.length)
    arr.reduce((pv, cv, ci, arr) => {
        if(pv < cv) {
            higher++
        }
        return cv
    })
    console.log(higher)
  })