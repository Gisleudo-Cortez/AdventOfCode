from pathlib import Path
from typing import Dict, Optional, Tuple

class Circuit:
    def __init__(self):
        self.wire_instructions = {}
        self.wire_cache = {}
    
    def add_instruction(self, instruction: str) -> None:
        """Parse and add a single instruction to the circuit."""
        parts = instruction.split(' -> ')
        if len(parts) != 2:
            raise ValueError(f"Invalid instruction format: {instruction}")
        
        target_wire = parts[1].strip()
        source = parts[0].strip().split()
        
        self.wire_instructions[target_wire] = source
    
    def override_wire(self, wire: str, value: int) -> None:
        """Override a wire's instruction with a direct value."""
        self.wire_instructions[wire] = [str(value)]
    
    def reset_cache(self) -> None:
        """Clear the wire value cache."""
        self.wire_cache = {}
        
    def get_wire_value(self, wire: str) -> int:
        """
        Calculate the value for a given wire, handling dependencies recursively.
        Returns a 16-bit unsigned integer.
        """
        # Return cached value if available
        if wire in self.wire_cache:
            return self.wire_cache[wire]
        
        # If wire is a number, return it
        if wire.isdigit():
            return int(wire)
        
        # Get the instruction for this wire
        instruction = self.wire_instructions[wire]
        
        # Calculate wire value based on instruction type
        if len(instruction) == 1:
            # Direct assignment
            value = self.get_wire_value(instruction[0])
        
        elif len(instruction) == 2:
            # NOT operation
            if instruction[0] != 'NOT':
                raise ValueError(f"Invalid unary operation: {instruction}")
            value = ~self.get_wire_value(instruction[1]) & 0xFFFF
        
        elif len(instruction) == 3:
            # Binary operations
            left = self.get_wire_value(instruction[0])
            op = instruction[1]
            right = self.get_wire_value(instruction[2])
            
            if op == 'AND':
                value = left & right
            elif op == 'OR':
                value = left | right
            elif op == 'LSHIFT':
                value = (left << right) & 0xFFFF
            elif op == 'RSHIFT':
                value = (left >> right) & 0xFFFF
            else:
                raise ValueError(f"Invalid binary operation: {op}")
        
        else:
            raise ValueError(f"Invalid instruction format: {instruction}")
        
        # Cache the calculated value
        self.wire_cache[wire] = value
        return value

def solve_part1(instructions: list[str]) -> int:
    """Solve part 1: Get the signal on wire 'a'."""
    circuit = Circuit()
    for instruction in instructions:
        circuit.add_instruction(instruction)
    return circuit.get_wire_value('a')

def solve_part2(instructions: list[str], part1_result: int) -> int:
    """Solve part 2: Override wire 'b' with part1 result and recalculate wire 'a'."""
    circuit = Circuit()
    
    # Add all instructions
    for instruction in instructions:
        circuit.add_instruction(instruction)
    
    # Override wire 'b' with the signal from part 1
    circuit.override_wire('b', part1_result)
    
    # Calculate new value for wire 'a'
    return circuit.get_wire_value('a')

def main():
    # Read input data
    data = Path("../input.txt").read_text().strip().splitlines()
    
    # Solve part 1
    part1_result = solve_part1(data)
    print(f"Part 1 - Wire 'a' value: {part1_result}")
    
    # Solve part 2
    part2_result = solve_part2(data, part1_result)
    print(f"Part 2 - Wire 'a' new value: {part2_result}")

if __name__ == "__main__":
    main()