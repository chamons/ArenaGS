using System;
using AppKit;
using ArenaGS.Platform;
using Foundation;

namespace ArenaGS.Mac
{
	[Register ("AppDelegate")]
	public class AppDelegate : NSApplicationDelegate
	{

		public override void DidFinishLaunching (NSNotification notification)
		{
			ObjCRuntime.Runtime.MarshalManagedException += (sender, args) =>
			{
				ILogger log = Dependencies.Get<ILogger> ();
				Exception e = args.Exception;
				log.Log ($"Uncaught exception \"{e.Message}\" with stacktrace:\n {e.StackTrace}. Exiting.", LogMask.All, Servarity.Normal);
				throw e;
			};
		}

		public override bool ApplicationShouldTerminateAfterLastWindowClosed (NSApplication sender)
		{
			return true;
		}
	}
}
