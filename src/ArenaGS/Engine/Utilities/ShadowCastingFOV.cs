/* Derived from fov_recursive_shadowcasting.c
* libtcod 1.6.3
* Copyright (c) 2008,2009,2010,2012,2013,2016,2017 Jice & Mingos & rmtew
* All rights reserved.
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*     * Redistributions of source code must retain the above copyright
*       notice, this list of conditions and the following disclaimer.
*     * Redistributions in binary form must reproduce the above copyright
*       notice, this list of conditions and the following disclaimer in the
*       documentation and/or other materials provided with the distribution.
*     * The name of Jice or Mingos may not be used to endorse or promote products
*       derived from this software without specific prior written permission.
*
* THIS SOFTWARE IS PROVIDED BY JICE, MINGOS AND RMTEW ``AS IS'' AND ANY
* EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
* WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL JICE, MINGOS OR RMTEW BE LIABLE FOR ANY
* DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
* (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
* LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
* ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
* (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
* SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

using System;
using System.Linq;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Engine.Utilities
{
    internal static class ShadowCastingFOV
	{
		static int [,] mult = new int [4, 8] {{1,0,0,-1,-1,0,0,1},
													{0,1,-1,0,0,-1,1,0},
													{0,1,1,0,0,-1,-1,0},
													{1,0,0,1,-1,0,0,-1}};

		static void CastLight (GameState state, MapVisibility visibilityInfo, int cx, int cy, int row, float start, float end, int radius, int r2, int xx, int xy, int yx, int yy, int id, bool light_walls)
		{
			float new_start = 0.0f;
			if (start < end)
				return;

			for (int j = row; j < radius + 1; j++)
			{
				int dx = -j - 1;
				int dy = -j;
				bool blocked = false;
				while (dx <= 0)
				{
					dx++;
					int X = cx + dx * xx + dy * xy;
					int Y = cy + dx * yx + dy * yy;
					if ((uint)X < (uint)visibilityInfo.Width && (uint)Y < (uint)visibilityInfo.Height)
					{
						int offset = X + Y * visibilityInfo.Width;
						float l_slope = (dx - 0.5f) / (dy + 0.5f);
						float r_slope = (dx + 0.5f) / (dy - 0.5f);
						if (start < r_slope)
							continue;
						else if (end > l_slope)
							break;
						if (dx * dx + dy * dy <= r2 && (light_walls || state.Map [X, Y].Transparent))
							visibilityInfo.Visibility [X, Y] = true;
						if (blocked)
						{
							if (!state.Map [X, Y].Transparent)
							{
								new_start = r_slope;
								continue;
							}
							else
							{
								blocked = false;
								start = new_start;
							}
						}
						else
						{
							if (!state.Map [X, Y].Transparent && j < radius)
							{
								blocked = true;
								CastLight (state, visibilityInfo, cx, cy, j + 1, start, l_slope, radius, r2, xx, xy, yx, yy, id + 1, light_walls);
								new_start = r_slope;
							}
						}
					}
				}
				if (blocked)
					break;
			}
		}

		internal static MapVisibility ComputeRecursiveShadowcasting (GameState state, Character character, int maxRadius, bool lightWalls)
		{
			MapVisibility visibilityInfo = new MapVisibility (state.Map.Width, state.Map.Height);

			if (maxRadius == 0)
			{
				int max_radius_x = visibilityInfo.Width - character.Position.X;
				int max_radius_y = visibilityInfo.Height - character.Position.Y;
				max_radius_x = Math.Max (max_radius_x, character.Position.X);
				max_radius_y = Math.Max (max_radius_y, character.Position.Y);
				maxRadius = (int)(Math.Sqrt (max_radius_x * max_radius_x + max_radius_y * max_radius_y)) + 1;
			}
			int r2 = maxRadius * maxRadius;
			/* recursive shadow casting */
			for (int oct = 0; oct < 8; oct++)
			{
				CastLight (state, visibilityInfo, character.Position.X, character.Position.Y, 1, 1.0f, 0.0f, maxRadius, r2, mult [0, oct], mult [1, oct], mult [2, oct], mult [3, oct], 0, lightWalls);
			}
			visibilityInfo.Visibility [character.Position.X, character.Position.Y] = true;
			return visibilityInfo;
		}
	}
}
