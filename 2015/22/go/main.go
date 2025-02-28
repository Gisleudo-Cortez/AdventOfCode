package main

import (
	"container/heap"
	"fmt"
	"math"
	"sort"
	"strings"
)

type Spell struct {
	name     string
	cost     int
	damage   int
	heal     int
	armor    int
	manaGain int
	duration int
}

var SPELLS = []Spell{
	{"Magic Missile", 53, 4, 0, 0, 0, 0},
	{"Drain", 73, 2, 2, 0, 0, 0},
	{"Shield", 113, 0, 0, 7, 0, 6},
	{"Poison", 173, 3, 0, 0, 0, 6},
	{"Recharge", 229, 0, 0, 0, 101, 5},
}

type GameState struct {
	playerHP      int
	playerMana    int
	bossHP        int
	bossDamage    int
	activeEffects map[string]int
	manaSpent     int
	hardMode      bool
	armor         int
}

// applyEffects applies all active spell effects to the game state.
func (gs *GameState) applyEffects() {
	gs.armor = 0
	toRemove := []string{}
	for _, spell := range SPELLS {
		if duration, ok := gs.activeEffects[spell.name]; ok {
			if spell.damage > 0 {
				gs.bossHP -= spell.damage
			}
			if spell.armor > 0 {
				gs.armor = spell.armor
			}
			if spell.manaGain > 0 {
				gs.playerMana += spell.manaGain
			}
			duration--
			if duration == 0 {
				toRemove = append(toRemove, spell.name)
			} else {
				gs.activeEffects[spell.name] = duration
			}
		}
	}
	for _, name := range toRemove {
		delete(gs.activeEffects, name)
	}
}

// getValidSpells returns all spells that can be cast (affordable and not already active).
func (gs *GameState) getValidSpells() []*Spell {
	valid := []*Spell{}
	for i := range SPELLS {
		spell := &SPELLS[i]
		if spell.cost <= gs.playerMana {
			if _, active := gs.activeEffects[spell.name]; !active {
				valid = append(valid, spell)
			}
		}
	}
	return valid
}

// cloneState creates a deep copy of the given GameState.
func cloneState(gs *GameState) *GameState {
	newEffects := make(map[string]int)
	for k, v := range gs.activeEffects {
		newEffects[k] = v
	}
	return &GameState{
		playerHP:      gs.playerHP,
		playerMana:    gs.playerMana,
		bossHP:        gs.bossHP,
		bossDamage:    gs.bossDamage,
		activeEffects: newEffects,
		manaSpent:     gs.manaSpent,
		hardMode:      gs.hardMode,
		armor:         gs.armor,
	}
}

// stateKey creates a unique string key for a GameState (used for memoization).
func stateKey(gs *GameState) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("%d,%d,%d,%t;", gs.playerHP, gs.playerMana, gs.bossHP, gs.hardMode))
	// Sort the active effect keys to ensure consistent ordering.
	keys := make([]string, 0, len(gs.activeEffects))
	for k := range gs.activeEffects {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		sb.WriteString(fmt.Sprintf("%s:%d;", k, gs.activeEffects[k]))
	}
	return sb.String()
}

// stateHeap is a priority queue of GameState pointers ordered by manaSpent (ascending).
type stateHeap []*GameState

func (h stateHeap) Len() int { return len(h) }
func (h stateHeap) Less(i, j int) bool {
	return h[i].manaSpent < h[j].manaSpent
}
func (h stateHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}
func (h *stateHeap) Push(x interface{}) {
	*h = append(*h, x.(*GameState))
}
func (h *stateHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func findLeastManaToWin(playerHP, playerMana, bossHP, bossDamage int, hardMode bool) int {
	initialState := &GameState{
		playerHP:      playerHP,
		playerMana:    playerMana,
		bossHP:        bossHP,
		bossDamage:    bossDamage,
		activeEffects: make(map[string]int),
		manaSpent:     0,
		hardMode:      hardMode,
		armor:         0,
	}

	h := &stateHeap{}
	heap.Init(h)
	heap.Push(h, initialState)
	seen := make(map[string]int)

	for h.Len() > 0 {
		state := heap.Pop(h).(*GameState)

		// Hard mode: player loses 1 HP at the start of their turn.
		if state.hardMode {
			state.playerHP--
			if state.playerHP <= 0 {
				continue
			}
		}

		state.applyEffects()
		if state.bossHP <= 0 {
			return state.manaSpent
		}

		for _, spell := range state.getValidSpells() {
			newState := cloneState(state)
			newState.playerMana -= spell.cost
			newState.manaSpent += spell.cost

			if spell.duration > 0 {
				newState.activeEffects[spell.name] = spell.duration
			} else {
				newState.bossHP -= spell.damage
				newState.playerHP += spell.heal
			}

			if newState.bossHP <= 0 {
				return newState.manaSpent
			}

			newState.applyEffects()
			if newState.bossHP <= 0 {
				return newState.manaSpent
			}

			// Boss turn: apply damage.
			damageTaken := newState.bossDamage - newState.armor
			if damageTaken < 1 {
				damageTaken = 1
			}
			newState.playerHP -= damageTaken

			if newState.playerHP > 0 {
				key := stateKey(newState)
				if _, exists := seen[key]; !exists {
					seen[key] = newState.manaSpent
					heap.Push(h, newState)
				}
			}
		}
	}
	return math.MaxInt32
}

func main() {
	sep := strings.Repeat("=", 20)

	fmt.Printf("%s Part 1 %s\n", sep, sep)
	manaSpent := findLeastManaToWin(50, 500, 55, 8, false)
	fmt.Printf("Total mana spent: %d\n", manaSpent)

	fmt.Printf("%s Part 2 %s\n", sep, sep)
	manaSpent = findLeastManaToWin(50, 500, 55, 8, true)
	fmt.Printf("Total mana spent: %d\n", manaSpent)
}
