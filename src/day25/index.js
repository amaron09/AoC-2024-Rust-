/* Working JS solution */

const fs = require('fs');

const calculateHeight = (matrix, isKey) => {
  console.log("matrix x", matrix[1])
  let height = [0, 0, 0, 0, 0]

  /* if (isKey) {
    for (let y = 1; y < 6; y++) {
      for (let i = 0; i < 5; i++) {
        console.log("KEY", matrix[y][i])
        if (matrix[y][i] === '#') {
          height[i] = height[i] + 1
        }
      }
    }
  } else { */
    for (let y = 1; y < 6; y++) {
      for (let i = 0; i < 5; i++) {
        console.log("matrix[y][columnIndex]", matrix[y][i])
        if (matrix[y][i] === '#') {
          height[i] = height[i] + 1
        }
      }
    }
  /* } */
  return height
}

const vectorAddition = (vector1, vector2) => {
  return vector1.map((v, i) => v + vector2[i])
}

const checkFit = (vector1, vector2) => {
  const addition = vectorAddition(vector1, vector2)
  return addition.every((v) => v <= 5)
}

const main = () => {
  const data = fs.readFileSync('inputs/day25.txt', 'utf8');
  const blocks = data.split("\n\n");

  const items = blocks.reduce((acc, block) => {
    const item = block.split("\n");
    return [...acc, item];
  }, [])
  
  const keysAndLocks = items.reduce((acc, item, index) => {
    let isKey = false
    if (item[0].includes('.')) {
      isKey = true
    }

    const height = []

    const test = calculateHeight(item, isKey)

    if (isKey) {
      return {
        ...acc,
        keys: [...acc?.keys, test]
      }
    } else {
      return {
        ...acc,
        locks: [...acc?.locks, test]
      }
    }
  },{
    keys: [],
    locks: []
  })

  const uniqueKeys = keysAndLocks.keys.reduce((acc, curr) => {
    let fit = 0
    
    keysAndLocks.locks.forEach(lock => {
      const addition = checkFit(curr, lock)
      fit = fit + addition
      console.log("addition", addition)
    })

    return acc + fit
  }, 0)

  console.log("uniqueKeys", uniqueKeys)
}

main();