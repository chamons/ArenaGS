/* Derived from bresenham_c.c
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

using System.Collections.Generic;
using ArenaGS.Utilities;

namespace ArenaGS.Engine.Utilities
{
	// This code was taken from bresenham_c.c from libtcod
	// This code is except from many of the normal coding conversions
	// as it is a port of C code
	public class BresenhamLine
	{
		public static IEnumerable <Point> PointsOnLine (Point start, Point end)
		{
			List <Point> points = new List<Point> ();
			BresenhamLine line = new BresenhamLine (start, end);
			while (true)
			{
				Point nextStep = line.Step();
				if (nextStep == Point.Invalid)
					return points;
				points.Add (nextStep);
			}
		}
		
		int stepx;
		int stepy;
		int e;
		int deltax;
		int deltay;
		int origx;
		int origy;
		int destx;
		int desty;

		BresenhamLine (Point from, Point to)
		{
			origx = from.X;
			origy = from.Y;
			destx = to.X;
			desty = to.Y;
			deltax = to.X - from.X;
			deltay = to.Y - from.Y;
			if (deltax > 0)
			{
				stepx = 1;
			}
			else if (deltax < 0)
			{
				stepx = -1;
			}
			else stepx = 0;
			if (deltay > 0)
			{
				stepy = 1;
			}
			else if (deltay < 0)
			{
				stepy = -1;
			}
			else stepy = 0;
			if (stepx * deltax > stepy * deltay)
			{
				e = stepx * deltax;
				deltax *= 2;
				deltay *= 2;
			}
			else
			{
				e = stepy * deltay;
				deltax *= 2;
				deltay *= 2;
			}
		}

		Point Step()
		{
			if (stepx * deltax > stepy * deltay)
			{
				if (origx == destx) 
					return Point.Invalid;
				origx += stepx;
				e -= stepy * deltay;
				if (e < 0)
				{
					origy += stepy;
					e += stepx * deltax;
				}
			}
			else
			{
				if (origy == desty)
					return Point.Invalid;
				origy += stepy;
				e -= stepx * deltax;
				if (e < 0)
				{
					origx += stepx;
					e += stepy * deltay;
				}
			}
			return new Point(origx, origy);
		}
	}
}