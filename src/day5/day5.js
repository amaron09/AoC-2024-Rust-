const fs = require('fs');

const day5 = () => {
  const input = fs.readFileSync('inputs/day5.txt', 'utf8').split('\n');
/*   console.log("input", input) */
  const rules = []
  const updates = []

  input.forEach((value) => {
    console.log("value", value)
    if (value.includes('|')) {
      rules.push(value.split('|'))
    }

    if (value.includes(',')) {
      updates.push(value.split(','))
    }
  })

  updates.forEach((update) => {
    for (let i = 0; i < update.length; i++) {
      rules.forEach((rule) => {
        if (rule.includes(update[i])) {
          console.log("true", rule)
        }
      })
    }
    console.log("update", update)
  })
}

day5()