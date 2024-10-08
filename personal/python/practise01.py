import random
print('じゃんけん大会')

win = draw = lose = 0
i = 0
result = []

while True:
    i = i + 1
    print('じゃんけん', i, '回目')

    print('グー：０')
    print('チョキ：１')
    print('パー：２')
    user = int(input('手を入力してください。\n'))

    if user == 0:
        print('あなたの選んだ手はグーです。')
    elif user == 1:
        print('あなたの選んだ手はチョキです。')
    elif user == 2:
        print('あなたの選んだ手はパーです。')

    cpu = random.randint(0,2)
    if cpu == 0:
        print('cpuの選んだ手はグーです。')
    elif cpu == 1:
        print('cpuの選んだ手はチョキです。')
    elif cpu == 2:
        print('cpuの選んだ手はパーです。')

    if user - cpu == -1 or user - cpu == 2:
        print('You win!')
        win = win + 1
        result.append('win')
    elif user - cpu == 0:
        print('Draw')
        draw = draw + 1
        result.append('draw')
    elif user - cpu == 1 or user - cpu == -2:
        print('You lose!')
        lose = lose + 1
        result.append('lose')
    
    game = input('ゲームを続けますか？(y/n)\n')
    if game == 'n':
        break

print('win:',win,'draw:',draw,'lose:',lose)
print(result)