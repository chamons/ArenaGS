﻿using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

namespace ArenaGS
{
	internal class CharacterLibrary
	{
		Dictionary<string, Character> Characters = new Dictionary<string, Character> ();

		Generator Generator;

		public CharacterLibrary (Generator generator)
		{
			Generator = generator;

			Character player = Generator.CreateRawPlayer (Point.Empty, new Health (15), new Defense (1));
			player = player.WithSkills (new Skill[] {
				Generator.CreateSkill ("Aimed Shot"),
				Generator.CreateSkill ("Dash"),
				Generator.CreateSkill ("Point Blank Shot"),
				Generator.CreateSkill ("Move & Shoot"),
			}.ToImmutableList ());
			AddToLibrary (player);

			Character wolf = Generator.CreateRawCharacter ("Wolf", Point.Empty, new Health (12), new Defense (0));
			wolf = wolf.WithSkills (new Skill [] {
				Generator.CreateSkill ("Attack", "Bite"),
				Generator.CreateSkill ("Charge"),
			}.ToImmutableList ());
			AddToLibrary (wolf);

			Character skeleton = Generator.CreateRawCharacter ("Skeleton", Point.Empty, new Health (15), new Defense (1));
			skeleton = skeleton.WithSkills (new Skill [] {
				Generator.CreateSkill ("Attack", "Slash"),
				Generator.CreateSkill ("Stunning Strike"),
			}.ToImmutableList ());
			AddToLibrary (skeleton);

			Character skeletonArcher = Generator.CreateRawCharacter ("Skeleton Archer", Point.Empty, new Health (8), new Defense (0));
			skeletonArcher = skeletonArcher.WithSkills (new Skill [] {
				Generator.CreateSkill ("Aimed Shot"),
			}.ToImmutableList ());
			AddToLibrary (skeletonArcher);

			Character golem = Generator.CreateRawCharacter ("Golem", Point.Empty, new Health (24), new Defense (0));
			golem = golem.WithSkills (new Skill [] {
				Generator.CreateSkill ("Attack", 1, "Slam"),
				Generator.CreateSkill ("Power Slam"),
			}.ToImmutableList ());
			AddToLibrary (golem);

			Character testEnemy = Generator.CreateRawCharacter ("TestEnemy", Point.Empty, new Health (1), new Defense (0));
			AddToLibrary (testEnemy);

			Character testPlayer = Generator.CreateRawPlayer (Point.Empty, new Health (10), new Defense (1));
			AddToLibrary (testPlayer, "TestPlayer");
		}

		void AddToLibrary (Character c, string name)
		{
			Characters.Add (name, c);
		}

		void AddToLibrary (Character c)
		{
			Characters.Add (c.Name, c);
		}

		public Character CreateCharacter (string name)
		{
			Character character;
			if (Characters.TryGetValue (name, out character))
				return character;
			throw new ArgumentException ($"Unknown character {name} in library");
		}
	}
}
