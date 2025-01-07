# from pathlib import Path
# import hashlib

# data = Path("../input.txt").read_text().strip()

# def brute_force_check(data:str, n_zeroes:int = 5) -> str:
#     answer = 0
#     base_data = data.encode()
#     while True:
#         test_str = base_data + str(answer).encode()
#         md5 = hashlib.md5(test_str).hexdigest()
#         if md5.startswith("0" * n_zeroes):
#             return answer
#         answer += 1

# def main() -> None:
#     print(f"part 1: ", brute_force_check(data, 5))
#     print(f"part 2: ", brute_force_check(data, 6))

# if __name__ == "__main__":
#     main()

# Optimized code

from pathlib import Path
import hashlib
from multiprocessing import Pool, cpu_count
import itertools

def check_hash(args):
    data, start_num, batch_size, n_zeroes = args
    base_data = data.encode()
    target = "0" * n_zeroes
    
    for i in range(start_num, start_num + batch_size):
        test_str = base_data + str(i).encode()
        if hashlib.md5(test_str).hexdigest().startswith(target):
            return i
    return None

def brute_force_check(data: str, n_zeroes: int = 5) -> int:
    num_processes = cpu_count()
    batch_size = 100_000
    
    with Pool(processes=num_processes) as pool:
        for batch_start in itertools.count(0, batch_size * num_processes):
            tasks = [
                (data, batch_start + i * batch_size, batch_size, n_zeroes)
                for i in range(num_processes)
            ]
            
            results = pool.map(check_hash, tasks)
            
            for result in results:
                if result is not None:
                    return result

def main() -> None:
    data = Path("../input.txt").read_text().strip()
    
    print(f"Part 1: {brute_force_check(data, 5)}")
    print(f"Part 2: {brute_force_check(data, 6)}")

if __name__ == "__main__":
    main()