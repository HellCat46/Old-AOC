file = open('data/rps.txt', 'r')
f = file.readlines()

score = 0
rpsArray = [
    'BX','CX','AX', #Loss
    'AY','BY','CY', #Ties
    'CZ','AZ','BZ' #Wins
]
rounds = 0;
for line in f:
    rounds += 1
    line = line.strip().replace(" ", "")
    solution = rpsArray.index(line)
    if solution > 2 and solution <= 5:
        score += 3
    elif solution > 5:
        score += 6

    score += (solution % 3) + 1


print(score)
print(rounds)
