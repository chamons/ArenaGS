using System;
using System.Diagnostics;
using System.Threading.Tasks;

namespace ArenaGS.Views.Utilities
{
	internal class AnimationHelper
	{
		internal AnimationHelper ()
		{
			Reset();
		}

		public bool AnimationInProgress;
		public int Frame;
		int Length;
		Stopwatch Stopwatch;
		Action OnAnimationComplete;

		internal async Task AnimationLoop (int length, Action invalidateProc, Action onAnimationComplete)
		{
			OnAnimationComplete = onAnimationComplete;;
			Length = length;

			AnimationInProgress = true;
			Stopwatch.Start();

			while (AnimationInProgress)
			{
				Frame++;
				if (Frame > length)
				{
					AnimationInProgress = false;
					onAnimationComplete ();
					return;
				}

				invalidateProc ();
				await Task.Delay (TimeSpan.FromSeconds (1.0 / 30));
			}

			Stopwatch.Stop();
		}

		internal void Reset ()
		{
			AnimationInProgress = false;
			Frame = 0;
			Stopwatch = new Stopwatch ();
		}

		internal float PercentComplete => (float)Frame / (float)Length;
	}
}
