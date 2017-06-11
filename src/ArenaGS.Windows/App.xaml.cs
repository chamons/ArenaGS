using ArenaGS.Platform;
using System;
using System.Windows;

namespace ArenaGS.Windows
{
	/// <summary>
	/// Interaction logic for App.xaml
	/// </summary>
	public partial class App : Application
	{
		public App ()
		{
			AppDomain.CurrentDomain.UnhandledException += (o, e) =>
			{
				ILogger log = Dependencies.Get<ILogger> ();
				Exception exception = e.ExceptionObject as Exception;
				log.Log ($"Uncaught exception \"{exception.Message}\" with stacktrace:\n {exception.StackTrace}. Exiting.", LogMask.All, Servarity.Normal);
				throw exception;
			};
		}
	}
}
