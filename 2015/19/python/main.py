from pathlib import Path
from typing import Tuple, Dict, List
from collections import defaultdict

def load(path: str) -> Tuple[List[str], List[str], str]:
    rules, molecule = Path(path).read_text().strip().split("\n\n")
    rules = [x.split(" => ") for x in rules.splitlines()]
    rk = [rule[0] for rule in rules]
    rv = [rule[1] for rule in rules]
    return rk, rv, molecule

def gen_replacements(molecule: str, rk: List[str], rv: List[str]) -> int:
    molecules = set()
    for i, r in enumerate(rk):
        rep = rv[i]
        start_ind = 0
        while (index := molecule.find(r, start_ind)) != -1:
            n_mol = molecule[:index] + rep + molecule[index + len(r):]
            molecules.add(n_mol)
            start_ind = index + 1
    return len(molecules)

def gen_from_e(molecule: str, rk: List[str], rv: List[str]) -> int:
    steps = 0
    current = molecule
    while current != "e":
        for i, rep in enumerate(rv):
            if rep in current:
                n_mol = current.replace(rep, rk[i], 1)
                current = n_mol
                steps += 1
                break
        else:
            return -1
    return steps

def main() -> None:
    rk, rv, molecule = load("../input.txt")
    print("=" * 20, "Part 1", "=" * 20)
    n_unique = gen_replacements(molecule, rk, rv)
    print(f"The number of unique combinations is: {n_unique}")
    print("=" * 20, "Part 2", "=" * 20)
    n_steps = gen_from_e(molecule, rk, rv)
    print(f"The number of steps required is: {n_steps}")

if __name__ == "__main__":
    main()