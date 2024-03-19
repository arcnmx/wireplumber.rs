local args = ...
if args then
	args = args:parse()
	Debug.dump_table(args)
else
	print(args)
end
Core.quit()
