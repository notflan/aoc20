#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#include <attrs.h>
#include <panic.h>

const char* const input[] = {
#ifdef TEST
#include "input-test.h"
#else
#include "input.h"
#endif
};

#define input_sz (sizeof(input)/sizeof(char*))

enum op {
	OP_NOP=0, 
	OP_ACC,
	OP_JMP,
	OP_HLT,
};

inline static const char* strop(enum op op)
{
	switch(op) {
		case OP_NOP: return "nop";
		case OP_ACC: return "acc";
		case OP_JMP: return "jmp";
		case OP_HLT: return "hlt";
	}
	panic("Unknown op %d", op);
}

enum exec_stat {
	END = 0,
	HALT,
};

typedef struct instr {
	enum op operator;
	int64_t operand;
} instruction_t;

struct reg {
	ssize_t pc;
	int64_t acc;
	int pjmp[input_sz];
};


static pure instruction_t parse_single(const char* str)
{
	register instruction_t op;
	switch(str[0]) {
		case 'n': op.operator = OP_NOP; break;
		case 'a': op.operator = OP_ACC; break;
		case 'j': op.operator = OP_JMP; break;
		default: panic("Unknown instr op: %s", str);
	}
	op.operand = (int64_t)atoll(str+4);
	return op;
}

enum exec_stat execute_destructive(instruction_t* restrict prog, struct reg* restrict reg, size_t len);

enum exec_stat execute(const instruction_t* prog, struct reg* restrict reg)
{
	instruction_t dest[input_sz];
	memcpy(dest, prog, input_sz * sizeof(prog[0]));
	reg->pc = 0;
	reg->acc =0;
	return execute_destructive(dest, reg, input_sz);
}
enum exec_stat execute_destructive(instruction_t* restrict prog, struct reg* restrict reg, size_t len)
{
	for(;(size_t)reg->pc<len;)
	{
		if(reg->pc < 0) panic("Invalid pc %ld", reg->pc);

		instruction_t* restrict current = &prog[reg->pc];
		enum op op = current->operator;
		current->operator = OP_HLT;
		switch(op)
		{
			case OP_HLT: return HALT;
			case OP_JMP: 
				reg->pc += (ssize_t)current->operand;
				continue;
			case OP_ACC: reg->acc += current->operand; break;
			default: break;
		}
		reg->pc += 1;
	}

	return END;
}

void swap(size_t* restrict s1, size_t* restrict s2)
{
	*s1 = *s1 ^ *s2;
	*s2 = *s1 ^ *s2;
	*s1 = *s1 ^ *s2;
}

void reverse(size_t* restrict ar, size_t len)
{
	size_t i=0,j=len-1;
	if(j==1) return;
	while(i<j) {
		
		swap(ar+i, ar+j);
		i++; j--;
	}
}

int main()
{
	instruction_t instr[input_sz];
	size_t jumps[input_sz]; size_t nj =0;
	size_t nops[input_sz]; size_t ni=0;
	for (size_t i=0;i<input_sz;i++) {
		switch((instr[i] = parse_single(input[i])).operator)
		{
			case OP_NOP: nops[ni++] = i; break;
			case OP_JMP: jumps[nj++] = i; break;
			default: break;
		}
	}
	reverse(jumps, nj);
	reverse(nops, ni);
	
	struct reg state = {0};
#ifdef PART2
	register int seti=0,setj=0;
	while (execute(instr, &state)) {
		if(seti)instr[nops [ni]].operator = OP_NOP;
		if(setj)instr[jumps[nj]].operator = OP_JMP;
		
		seti=setj=0;
		if(ni) {
			instr[nops [--ni]].operator = OP_JMP;
			seti=1;
		}
		else if(nj) {
			instr[jumps[--nj]].operator = OP_NOP;
			setj=1;
		}
		else panic("All inputs halt: %ld", state.acc);
#ifdef DEBUG
		fprintf(stderr, "Retry: %ld %ld\n", state.pc, state.acc);
#endif
	}
#else
	enum exec_stat res = execute_destructive(instr, &state, input_sz);
	if(!res) panic("No halt");	
#endif
	printf("%ld\n", state.acc);
	return 0;
}

