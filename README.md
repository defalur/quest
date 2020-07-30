# quest

This is a simple project that aims to generate quests for a role-playing game randomly.

This project is split in three main parts:

## Name generation
A language is generated in order to generate names that look coherent.

The method used for name generation is loosely based on the following article: <https://mewo2.com/notes/naming-language/>

## People, location and mob generation
The language is used to randomly generate characters, mobs and locations in the world.

All of these are stored in a big world struct that contains the language data, and all generated objects.

## Quest generation
The quest generation uses the world data to generate quests. It selects people, the type of goals, items, and locations randomly and outputs a quest struct.
