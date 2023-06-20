# Cemantix cheat

Cemantix cheat aims to discover the word of the day automatically.
The cemantix game can be found at https://cemantix.certitudes.org/

## Requirements

- Chromedriver: Find the driver fitting your chrome version at https://chromedriver.chromium.org/ ; and add chromedriver to your PATH.
- frWac words worpus: can be found at https://fauconnier.github.io/#data ; I personnally use the second available (120Mb), named frWac_non_lem_no_postag_no_phrase_200_cbow_cut100.bin ; add the bin file to /api once downloaded.

## Run

In order to run, use the run.sh script. 
When executing for the first time, use the -f option to install required python packages and build the project.

## Key libraries & concepts

- Selenium: browser automation
- Gensim keyedvectors: algorithm used to calculate the most similar words when inputting one specific word
- Lemmatisation: tranforming a word to get its root word (removing plurals, conjugated verbs and nouns in feminine version as they are not taken into account by the game)
- Tokyo: working async

## Configuration

The configuration file is config.json, and enable the configuration of:
- Every xpaths and html ids
- The chromdriver and API urls
- The path to the starting words document
- The randomization of the order of the starting words (ON/OFF) 
