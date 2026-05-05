package blockstore

// registerMediaSpecs seeds media-asset block types: image, video, file,
// embed, bookmark, audio, equation, chart, synced_block. Each carries a
// `url` or `source_id` pointing at external content; renderer decides the
// actual embed/iframe strategy based on `provider`.
func registerMediaSpecs(m map[string]BlockTypeSpec) {
	m[BlockTypeImage] = BlockTypeSpec{
		Type:            BlockTypeImage,
		DefaultView:     "document",
		SupportedViews:  []string{"document", "gallery"},
		// data: {url: string, alt?: string, width?: number, height?: number}
		RequiredDataKey: []string{"url"},
		AllowedChildren: []string{},
	}
	m[BlockTypeFile] = BlockTypeSpec{
		Type:            BlockTypeFile,
		DefaultView:     "document",
		SupportedViews:  []string{"document", "list"},
		// data: {url: string, name: string, size?: number, mime?: string}
		RequiredDataKey: []string{"url", "name"},
		AllowedChildren: []string{},
	}
	m[BlockTypeVideo] = BlockTypeSpec{
		Type:            BlockTypeVideo,
		DefaultView:     "document",
		SupportedViews:  []string{"document"},
		// data: {url: string, provider?: "native"|"youtube"|"vimeo"}
		RequiredDataKey: []string{"url"},
		AllowedChildren: []string{},
	}
	m[BlockTypeEmbed] = BlockTypeSpec{
		Type:            BlockTypeEmbed,
		DefaultView:     "document",
		SupportedViews:  []string{"document"},
		// data: {url: string, provider?: string}
		RequiredDataKey: []string{"url"},
		AllowedChildren: []string{},
	}
	m[BlockTypeBookmark] = BlockTypeSpec{
		Type:            BlockTypeBookmark,
		DefaultView:     "document",
		SupportedViews:  []string{"document"},
		// data: {url: string, title?: string, description?: string, image?: string}
		RequiredDataKey: []string{"url"},
		AllowedChildren: []string{},
	}
	m[BlockTypeAudio] = BlockTypeSpec{
		Type:            BlockTypeAudio,
		DefaultView:     "document",
		SupportedViews:  []string{"document"},
		// data: {url: string, title?: string}
		RequiredDataKey: []string{"url"},
		AllowedChildren: []string{},
	}
	m[BlockTypeEquation] = BlockTypeSpec{
		Type:            BlockTypeEquation,
		DefaultView:     "document",
		SupportedViews:  []string{"document"},
		// data: {latex: string, display?: "inline"|"block"}
		RequiredDataKey: []string{"latex"},
		AllowedChildren: []string{},
	}
	m[BlockTypeChart] = BlockTypeSpec{
		Type:           BlockTypeChart,
		DefaultView:    "document",
		SupportedViews: []string{"document"},
		// data: {type: "bar"|"line"|"pie"|"area"|"scatter"|"radar",
		//        title?: string, x_key?: string, y_key?: string,
		//        x_label?: string, y_label?: string,
		//        series: [{name: string, color?: string, data: [...]}]}
		// pie uses data: [{name, value}]; scatter uses data: [{x, y}];
		// radar uses data: [{axis, value}] with x_key = axis field name.
		RequiredDataKey: []string{"type", "series"},
		EnumValues: map[string][]string{
			"type": {"bar", "line", "pie", "area", "scatter", "radar"},
		},
		NonEmptyArrayKeys: []string{"series"},
		AllowedChildren:   []string{},
	}
	m[BlockTypeSyncedBlock] = BlockTypeSpec{
		Type:            BlockTypeSyncedBlock,
		DefaultView:     "document",
		SupportedViews:  []string{"document"},
		// data: {source_id: string} — renderer resolves target block at read time
		RequiredDataKey: []string{"source_id"},
		AllowedChildren: []string{},
	}
}
