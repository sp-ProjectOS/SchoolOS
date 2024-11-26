import { TableColumn, TableMode } from '@/stores/Statistics'
import type { ColumnDef } from '@tanstack/vue-table'
import { h } from 'vue'
import { i18n } from '@/main'


const getTranslation = (key: string) => {
	const { t } = i18n.global
	return t(`table.${key}`)
}
