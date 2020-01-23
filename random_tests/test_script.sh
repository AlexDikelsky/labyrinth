#!/bin/bash

R --slave < /home/alex/games/labyrinth/random_tests/try_rand_directions.r | tr ' ' '\n' | /home/alex/games/labyrinth/target/debug/labyrinth
