from pathlib import Path
from typing import List, Tuple, Dict

def hlf(r: int) -> int:
    return r // 2

def tpl(r: int) -> int:
    return r * 3

def inc(r: int) -> int:
    return r + 1

def jmp(v: int) -> int:
    return v

def jie(r: int, v: int) -> int:
    return v if r % 2 == 0 else 1

def jio(r: int, v: int) -> int:
    return v if r == 1 else 1

def execute(data: List[str], reg:Dict[str,int] = {"a": 0, "b": 0}) -> Tuple[int, int]:
    cmds = {
        "hlf": hlf,
        "tpl": tpl,
        "inc": inc,
        "jmp": jmp,
        "jie": jie,
        "jio": jio,
    }
    len_data = len(data)
    i = 0

    while 0 <= i < len_data:
        instructions = data[i].split()
        cmd = instructions[0]

        if cmd in {"hlf", "tpl", "inc"}:
            r = instructions[1]
            reg[r] = cmds[cmd](reg[r])
            i += 1
        elif cmd == "jmp":
            v = int(instructions[1])
            i += cmds[cmd](v)
        elif cmd == "jie":
            r = instructions[1].strip(",")
            v = int(instructions[2])
            i += cmds[cmd](reg[r], v)
        elif cmd == "jio":
            r = instructions[1].strip(",")
            v = int(instructions[2])
            i += cmds[cmd](reg[r], v)
        else:
            break

    return reg["a"], reg["b"]

def main() -> None:
    data = [x.strip() for x in Path("../input.txt").read_text().splitlines()]
    print("=" * 20, "Part 1", "=" * 20)
    a, b = execute(data) 
    print(f"register a:{a}, b:{b}")
    print("=" * 20, "Part 2", "=" * 20)
    a, b = execute(data, {"a":1, "b":0}) 
    print(f"register a:{a}, b:{b}")

if __name__ == "__main__":
    main()
