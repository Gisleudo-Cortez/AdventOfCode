# from pathlib import Path
# from functools import cache

# data = Path("../input.txt").read_text().strip()

# # The next password after abcdefgh is abcdffaa.
# example = Path("../example.txt").read_text().strip() 

# @cache
# def check_if_valid(data: str) -> bool:
#     alphabet_dict = {
#     'a': 0, 'b': 1, 'c': 2, 'd': 3, 'e': 4, 'f': 5, 'g': 6, 'h': 7,
#     'i': 8, 'j': 9, 'k': 10, 'l': 11, 'm': 12, 'n': 13, 'o': 14,
#     'p': 15, 'q': 16, 'r': 17, 's': 18, 't': 19, 'u': 20, 'v': 21,
#     'w': 22, 'x': 23, 'y': 24, 'z': 25
#     }
#     r1, r2, r3 = False, False, False # Rules
#     out_passwd = "........" # password is 8 chars long and always lower case
#     increasing_char_pos = []
#     pair_pos = []
#     forbiden_letter_pos = None
#     # check if password contains three increasing letters
#     for idx in range(len(data) - 2):
#         vone = alphabet_dict[data[idx]]
#         vtwo = alphabet_dict[data[idx + 1]]
#         vthree = alphabet_dict[data[idx + 2]]
#         if vtwo == vone + 1 and vthree == vtwo + 1:
#             r1 = True
#             increasing_char_pos = [idx, idx + 1, idx + 2]
#             break
#     # check if the password contains i,o or l, if contains it is invalid
#     forbiden = ['i', 'o', 'l']
#     for idx in range(len(data)):
#         if data[idx] in forbiden:
#             r2 = False
#             forbiden_letter_pos = idx
#             break
#         else:
#             r2 = True
#     # must contain two different pairs of non overlaping letters, like aa or bb
#     matches = set()
#     for idx in range(len(data) - 1):
#         if data[idx] == data[idx + 1]:
#             matches.add((data[idx],data[idx + 1]))
#         if len(matches) >= 2:
#             pair_pos = [idx, idx + 1]
#             r3 = True
#     return r1 and r2 and r3

# @cache
# def generate_password(data: str) -> str:
#     alphabet_dict = {
#     'a': 0, 'b': 1, 'c': 2, 'd': 3, 'e': 4, 'f': 5, 'g': 6, 'h': 7,
#     'i': 8, 'j': 9, 'k': 10, 'l': 11, 'm': 12, 'n': 13, 'o': 14,
#     'p': 15, 'q': 16, 'r': 17, 's': 18, 't': 19, 'u': 20, 'v': 21,
#     'w': 22, 'x': 23, 'y': 24, 'z': 25
#     }
#     valid = check_if_valid(data)
#     while not valid:
#         chars = list(data)
        
#         # Process from right to left
#         idx = len(chars) - 1
#         carry = True
        
#         # Continue as long as we need to carry over
#         while carry and idx >= 0:
#             if chars[idx] == 'z':
#                 chars[idx] = 'a'  # Wrap around
#                 carry = True  # Need to carry over to next position
#             else:
#                 # Get current letter's value and increment it
#                 current_value = alphabet_dict[chars[idx]]
#                 next_letter = chr(ord('a') + ((current_value + 1) % 26))
#                 chars[idx] = next_letter
#                 carry = False  # No need to carry over
#             idx -= 1
            
#         # Convert back to string
#         data = ''.join(chars)
#         valid = check_if_valid(data)
    
#     return data

# def main() -> None:
#     print("="*20," Part 1 ", "="*20)
#     password = generate_password(data)
#     print(f"Original: {data}, updated: {password}")
#     print("="*20," Part 2 ", "="*20)
#     prev = password
#     password = generate_password("hepxxzaa")
#     print(f"Original: {prev}, updated: {password}")


# if __name__ == "__main__":
#     main()
# OPTIMIZED CODE
from pathlib import Path
from functools import cache

# Pre-compute constants
FORBIDDEN = {'i', 'o', 'l'}
ALPHABET = 'abcdefghijklmnopqrstuvwxyz'
ALPHA_TO_NUM = {c: i for i, c in enumerate(ALPHABET)}
NUM_TO_ALPHA = {i: c for i, c in enumerate(ALPHABET)}

@cache
def has_increasing_straight(password: str) -> bool:
    """Check if password contains three consecutive increasing letters."""
    for i in range(len(password) - 2):
        if (ALPHA_TO_NUM[password[i + 1]] == ALPHA_TO_NUM[password[i]] + 1 and 
            ALPHA_TO_NUM[password[i + 2]] == ALPHA_TO_NUM[password[i]] + 2):
            return True
    return False

@cache
def has_two_pairs(password: str) -> bool:
    """Check if password contains two different non-overlapping pairs."""
    pairs = set()
    i = 0
    while i < len(password) - 1:
        if password[i] == password[i + 1]:
            pairs.add(password[i])
            i += 2
        else:
            i += 1
        if len(pairs) >= 2:
            return True
    return False

def increment_password(password: str) -> str:
    """Increment password string, skipping forbidden characters."""
    chars = list(password)
    i = len(chars) - 1
    
    while i >= 0:
        current_char = chars[i]
        next_char = NUM_TO_ALPHA[(ALPHA_TO_NUM[current_char] + 1) % 26]
        
        if next_char in FORBIDDEN:
            next_char = NUM_TO_ALPHA[(ALPHA_TO_NUM[next_char] + 1) % 26]
        
        chars[i] = next_char
        
        if current_char != 'z':
            break
        i -= 1
    
    return ''.join(chars)

def generate_password(password: str) -> str:
    """Generate the next valid password."""
    password = increment_password(password)
    
    while True:
        # Skip passwords with forbidden characters immediately
        if any(c in FORBIDDEN for c in password):
            # Jump to next valid position by replacing forbidden char and rest with 'a'
            idx = next(i for i, c in enumerate(password) if c in FORBIDDEN)
            password = (
                password[:idx] + 
                NUM_TO_ALPHA[(ALPHA_TO_NUM[password[idx]] + 1) % 26] + 
                'a' * (len(password) - idx - 1)
            )
            continue
            
        if has_increasing_straight(password) and has_two_pairs(password):
            return password
            
        password = increment_password(password)

def main() -> None:
    data = Path("../input.txt").read_text().strip()
    
    print("=" * 20, " Part 1 ", "=" * 20)
    password = generate_password(data)
    print(f"Original: {data}, updated: {password}")
    
    print("=" * 20, " Part 2 ", "=" * 20)
    prev = password
    password = generate_password("hepxxzaa")
    print(f"Original: {prev}, updated: {password}")

if __name__ == "__main__":
    main()