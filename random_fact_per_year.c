/* random_fact_per_year.c
 *
 * A tiny program that asks for a birth year and prints a random fact
 * about that year.
 *
 * Compile with:
 *     gcc -std=c11 -Wall -Wextra -o random_fact_per_year random_fact_per_year.c
 *
 * Run:
 *     ./random_fact_per_year
 */

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

/* ---------- Data structures ---------- */

/* One fact about a specific year */
typedef struct {
    int year;          /* The year the fact refers to */
    const char *text;  /* Human‑readable fact */
} YearFact;

/* A small static database of facts.
 * Feel free to add more entries or load them from a file. */
static const YearFact facts[] = {
    { 1969, "Apollo 11 landed on the Moon – the first humans set foot on another celestial body." },
    { 1970, "The first Earth Day was celebrated, marking the start of the modern environmental movement." },
    { 1971, "Intel released the first microprocessor, the 4004, ushering in the era of personal computing." },
    { 1980, "Mount St. Helens erupted in Washington, USA, causing massive destruction." },
    { 1989, "The Berlin Wall fell, symbolizing the end of the Cold War." },
    { 1991, "The World Wide Web became publicly available, changing how we share information." },
    { 1997, "The first successful cloning of a mammal (Dolly the sheep) was announced." },
    { 2001, "Wikipedia was launched, creating a massive, collaborative online encyclopedia." },
    { 2004, "Facebook was founded, beginning the modern social‑media era." },
    { 2007, "Apple released the first iPhone, revolutionizing mobile computing." },
    { 2010, "The first iPad was introduced, popularizing tablet computers." },
    { 2016, "The first detection of gravitational waves confirmed a major prediction of Einstein's theory." },
    { 2020, "The COVID‑19 pandemic reshaped global health, economics, and daily life." },
    /* Add more facts here */
};

/* Number of facts in the static array */
static const size_t fact_count = sizeof(facts) / sizeof(facts[0]);

/* ---------- Helper functions ---------- */

/* Return the number of facts that match a given year */
static size_t count_facts_for_year(int year)
{
    size_t count = 0;
    for (size_t i = 0; i < fact_count; ++i) {
        if (facts[i].year == year) {
            ++count;
        }
    }
    return count;
}

/* Return a random fact for the given year.
 * The caller must ensure that at least one fact exists for that year. */
static const char *random_fact_for_year(int year)
{
    /* Collect indices of matching facts */
    size_t matches[ fact_count ];
    size_t match_idx = 0;

    for (size_t i = 0; i < fact_count; ++i) {
        if (facts[i].year == year) {
            matches[match_idx++] = i;
        }
    }

    /* Choose a random index among the matches */
    size_t chosen = matches[ rand() % match_idx ];
    return facts[chosen].text;
}

/* ---------- Main program ---------- */

int main(void)
{
    int year;
    char line[64];

    /* Seed the random number generator */
    srand((unsigned)time(NULL));

    printf("Enter a birth year (e.g., 1995): ");
    if (!fgets(line, sizeof(line), stdin)) {
        fprintf(stderr, "Error reading input.\n");
        return EXIT_FAILURE;
    }

    /* Convert the input string to an integer */
    if (sscanf(line, "%d", &year) != 1) {
        fprintf(stderr, "Invalid year entered.\n");
        return EXIT_FAILURE;                                                                                  
    }                                                                                                         
                                                                                                              
    /* Look for facts about the entered year */                                                               
    size_t available = count_facts_for_year(year);                                                            
    if (available == 0) {                                                                                     
        printf("Sorry, I don't have any facts for the year %d.\n", year);                                     
        return EXIT_SUCCESS;                                                                                  
    }                                                                                                         
                                                                                                              
    /* Pick and display a random fact */                                                                      
    const char *fact = random_fact_for_year(year);                                                            
    printf("\nFact about %d:\n%s\n", year, fact);                                                             
                                                                                                              
    return EXIT_SUCCESS;                                                                                      
} 
