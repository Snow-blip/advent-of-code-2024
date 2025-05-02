import numpy as np

def check_safety(f:str) -> int:
    with open(f) as file:
        safe_count = 0
        while True:
            try:
                row = [int(s) for s in file.readline().strip().split(" ")]
            except ValueError:
                break
        
            if row[0]>row[-1]:
                row.reverse()
            #print(row)
            safe = True
            for i in range(len(row)-1):
                diff = row[i+1]-row[i]
                if diff <1 or diff>3:
                    safe = False
                    break
            #print(safe)
            if safe:
                safe_count += 1
    return safe_count

if __name__ == "__main__":
	example_answer = 2
	if (ans:=check_safety("./example")) == example_answer:
		print(f"The answer is {check_safety('./input')}")
	else:
		print(f"Expected answer to example is {example_answer}, got {ans}")