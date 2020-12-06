#include <vector>
#include <iostream>
#include <fstream>
#include <string>
#include <tuple>

#include <attrs.h>

using mod_table = std::vector<std::vector<bool> >;

mod_table read_input(const char* _input)
{
	std::ifstream input(_input);
	if(!input) {
		std::cerr << "Failed to open file: " << _input << std::endl;
		std::terminate();
	}
	mod_table output;

	std::string str;
	while(input)
	{
		if(!std::getline(input, str)) break;
		std::vector<bool> ln(str.size());
		int i=0;
		for(const auto& chr : str)
		{
			switch(chr)
			{
			case '#': ln[i] = true; fall;
			default:  i++; break;
			}
		}
		output.push_back(std::move(ln));
		str.clear();
	}
	return output;
}

template<typename V>
inline std::size_t wrapping_index(const V& vec, std::size_t i)
{
	return (i % vec.size());
}

std::size_t count_slope(const mod_table& table, std::size_t stepx, std::size_t stepy)
{
	std::size_t x=0;
	std::size_t r=0;
	for(std::size_t y=0;y<table.size();y+=stepy, x+=stepx)
	{
		r += table[y][wrapping_index(table[y], x)] ? 1 : 0;
	}
	return r;
}

int main()
{
	
	const mod_table table = read_input("input");
#ifdef PART2
	const auto paths = {
		std::tuple<std::size_t, std::size_t>(1, 1),
		std::tuple<std::size_t, std::size_t>(3, 1),
		std::tuple<std::size_t, std::size_t>(5, 1),
		std::tuple<std::size_t, std::size_t>(7, 1),
		std::tuple<std::size_t, std::size_t>(1, 2),
	};

	std::size_t r=1;
	for(const auto& [x, y] : paths)
	{
		r*= count_slope(table, x, y);
	}
#else
	auto r = count_slope(table, 3, 1); 
#endif

	std::cout << r << std::endl;
	return 0;
}
