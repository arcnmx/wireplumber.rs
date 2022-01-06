local args = ...

Log.info("hello from lua")
if args == nil then
	print("called with no args")
else
	print("called with args:")
	Debug.dump_table(args)
end
